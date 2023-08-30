package health

import (
	"fmt"
	"os"
	"path/filepath"
	"reflect"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/mitchellh/go-homedir"
)

// addCustomValidators sets up thcon-internal validators for
// github.com/go-playground/validator/v10.
func addCustomValidators(validate *goValidator.Validate) {
	mustRegisterValidator := func(tag string, fn goValidator.Func) {
		err := validate.RegisterValidation(tag, fn)
		if err != nil {
			panic(fmt.Errorf("unable to register custom validator %q %w", tag, err))
		}
	}
	mustRegisterValidator("expfile", expandedFile)
	mustRegisterValidator("sameExtAs", sameExtensionAs)
}

// expandedFile is identical to the default 'file' validator, but expands '~'
// in paths to platform-specific home directories first.
// See: https://github.com/go-playground/validator/blob/f5e5146b316bf0283fd8988dc9f26aebe94b599d/baked_in.go#L1473-L1488
func expandedFile(fl goValidator.FieldLevel) bool {
	field := fl.Field()
	if field.Kind() != reflect.String {
		panic(fmt.Sprintf("Bad field type %T", field.Interface()))
	}

	// Convert ~/foo/bar to a platform-specific home directory.
	expanded, err := homedir.Expand(field.String())
	if err != nil {
		return false
	}

	fileInfo, err := os.Stat(expanded)
	// An err that isn't os.ErrNotExist implies the file doesn't exist.
	// It may be unreadable, or any other FS error could have caused it.
	if err != nil {
		return false
	}

	// Disallow directories.
	return !fileInfo.IsDir()
}

// sameExtensionAs ensures that the tagged field and the named field both have
// the same file extension.
func sameExtensionAs(fl goValidator.FieldLevel) bool {
	field := fl.Field()
	if field.Kind() != reflect.String {
		panic(fmt.Sprintf("Bad field type %T", field.Interface()))
	}

	parent := fl.Parent()
	if parent.Kind() != reflect.Struct {
		panic(fmt.Sprintf("Bad field type %T", field.Interface()))
	}

	param := fl.Param()
	other := parent.FieldByName(param)
	if other == reflect.ValueOf(nil) {
		panic(fmt.Sprintf("Bad field name provided %q", param))
	}

	if other.Kind() != reflect.String {
		panic(fmt.Sprintf("Bad field type for %q %T", param, field.Interface()))
	}

	thisVal := field.String()
	otherVal := other.String()

	return filepath.Ext(thisVal) == filepath.Ext(otherVal)
}

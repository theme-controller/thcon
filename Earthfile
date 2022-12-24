VERSION 0.6
FROM golang:1.18.1-bullseye
WORKDIR /thcon

RUN apt update
RUN apt install -y libglib2.0-dev

deps:
  # Copy just enough to download dependencies
  COPY go.mod go.sum ./
  RUN go mod download

sources:
  FROM +deps

  COPY ./main.go ./
  COPY ./cmd ./cmd
  COPY ./lib ./lib

build:
  FROM +sources

  # Build it
  RUN go build

  # Save the output
  SAVE ARTIFACT thcon AS LOCAL thcon

test:
  FROM +sources

  # Run tests
  RUN go test ./...

protoc-gen:
  RUN apt install -y protobuf-compiler protoc-gen-go

  WORKDIR ./proto
  COPY ./misc/proto/* ./

  RUN protoc \
    --proto_path=. \
    --go_out=. \
    --go_opt=Miterm2.proto="github.com/theme-controller/thcon/lib/apps;iterm2pb",paths=source_relative \
    iterm2.proto

  SAVE ARTIFACT iterm2.pb.go AS LOCAL ./lib/apps/iterm2pb/iterm2pb_darwin.go

lint:
  FROM +sources
  RUN curl -sSfL https://raw.githubusercontent.com/golangci/golangci-lint/master/install.sh | sh -s -- -b $(go env GOPATH)/bin v1.50.1
  RUN golangci-lint run -v

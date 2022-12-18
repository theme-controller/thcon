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

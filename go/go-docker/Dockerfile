FROM golang:1.20.5-bullseye

WORKDIR /app

COPY go.mod ./
RUN go mod download

COPY *.go ./

RUN go build -o /go-docker

EXPOSE 8080

CMD [ "/go-docker" ]
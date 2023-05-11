FROM golang:1.19

WORKDIR /app

COPY go.mod go.sum ./ 

RUN go mod download # Download dependencies

COPY . .

RUN go build -o main . # Build the application

EXPOSE 5000

CMD ["./main"] # Run the application

package main

import (
	"database/sql"
	"log"
	"net/http"
	"os"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	_ "github.com/lib/pq"
)

func main() {

	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"*"}, // This is for allowing all origins
		AllowHeaders: []string{echo.HeaderOrigin, echo.HeaderContentType, echo.HeaderAccept},
		AllowMethods: []string{http.MethodGet, http.MethodPost, http.MethodPut, http.MethodDelete},
	}))

	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})

	e.GET("/health", func(c echo.Context) error {
		return c.String(http.StatusOK, "OK")
	})

	// Now for connecting the postgres
	e.GET("/postgres", postgresHandler)

	httpPort := os.Getenv("PORT")
	if httpPort == "" {
		httpPort = "5000"
	}

	e.Logger.Fatal(e.Start(":" + httpPort))
}

func dbConnector() *sql.DB {
	connStr := "postgres://admin:admin@local_pgdb:5432/postgres?sslmode=disable"
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		log.Fatal(err)
	}
	return db
}

func postgresHandler(c echo.Context) error {
	dbConnection := dbConnector()
	var res string
	// We are just querying the datbase version
	err := dbConnection.QueryRow("SELECT version()").Scan(&res)
	if err != nil {
		log.Fatal(err)
	}
	return c.String(http.StatusOK, res)
}

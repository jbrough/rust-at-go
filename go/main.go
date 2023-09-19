package main

import (
	"encoding/json"
	"log"
	"net/http"
)

func main() {
	addr := ":1338"
	http.HandleFunc("/", apiHandler)

	log.Printf("Go bench server listening on %s", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}

func apiHandler(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case "GET":
		if r.URL.Path == "/" {
			response := map[string]string{
				"status": "green",
			}
			body, err := json.Marshal(response)
			if err != nil {
				http.Error(w, "JSON serialization error", http.StatusInternalServerError)
				return
			}

			w.Header().Set("Content-Type", "application/json")
			w.Write(body)
			return
		}
	default:
		http.Error(w, "Not Found", http.StatusNotFound)
	}
}

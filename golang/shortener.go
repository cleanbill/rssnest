package main

import (
	"bytes"
	"encoding/json"
	"io"
	"log"
	"net/http"
)

type ShortURL struct {
	Kind    string
	Id      string
	LongURL string
	Err     error
}

type Shortener interface {
	Shorten(feedURL string, apiKey string) *ShortURL
}

type GoogleShort struct {
}

func (g GoogleShort) Shorten(feedURL string, apiKey string) *ShortURL {
	short := new(ShortURL)
	//curl https://www.googleapis.com/urlshortener/v1/url?key=blar -H 'Content-Type: application/json' -d '{"longUrl": "http://superuser.com/questions/149329/what-is-the-curl-command-line-syntax-to-do-a-post-request"}'
	client := &http.Client{}
	url := "https://www.googleapis.com/urlshortener/v1/url?key=" + apiKey
	var jsonStr = []byte(`{"longUrl":"` + feedURL + `"}`)
	req, _ := http.NewRequest("POST", url, bytes.NewBuffer(jsonStr))
	req.Header.Set("Content-Type", "application/json")
	response, err := client.Do(req)
	if err != nil {
		log.Printf("Cannot shorten, %v\n", err)
		short.Err = err
		return short
	}
	defer response.Body.Close()
	//ioutil.ReadAll starts at a very small 512
	//it really should let you specify an initial size
	buffer := bytes.NewBuffer(make([]byte, 0, 65536))
	io.Copy(buffer, response.Body)
	temp := buffer.Bytes()
	length := len(temp)
	var data []byte
	//are we wasting more than 10% space?
	if cap(temp) > (length + length/10) {
		data = make([]byte, length)
		copy(data, temp)
	} else {
		data = temp
	}
	json.Unmarshal(data, &short)
	return short
}

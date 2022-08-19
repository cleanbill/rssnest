package main

import (
	"bytes"
	"encoding/xml"
	"io"
	"log"
	"net/http"

	"github.com/bangbangsoftware/feeds"
)

type HttpSource struct {
}

func (s HttpSource) GetRSS(feedURL string) *feeds.Rss {
	r, err := http.Get(feedURL)
	if err != nil {
		log.Println(err)
		return nil
	}
	defer r.Body.Close()

	//ioutil.ReadAll starts at a very small 512
	//it really should let you specify an initial size
	buffer := bytes.NewBuffer(make([]byte, 0, 65536))
	io.Copy(buffer, r.Body)
	temp := buffer.Bytes()
	length := len(temp)
	var XMLdata []byte
	//are we wasting more than 10% space?
	if cap(temp) > (length + length/10) {
		XMLdata = make([]byte, length)
		copy(XMLdata, temp)
	} else {
		XMLdata = temp
	}

	buf := bytes.NewBuffer(XMLdata)
	decoded := xml.NewDecoder(buf)

	rss := new(feeds.Rss)
	err = decoded.Decode(rss)
	if err != nil {
		log.Println(err)
		//return nil, ok there's an error, so what lets go anyway
	}
	return rss
}

func (s HttpSource) DetectContentType(data []byte) string {
	return http.DetectContentType(data)
}

func (s HttpSource) Get(url string) (resp *http.Response, err error) {
	return http.Get(url)
}

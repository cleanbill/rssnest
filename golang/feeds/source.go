package feeds

import (
	"net/http"
)

type Rss struct {
	Channel Channel `xml:"channel"`
}

type Enclosure struct {
	Url string `xml:"url,attr"`
}

type RssItem struct {
	Title       string    `xml:"title"`
	Link        string    `xml:"link"`
	Description string    `xml:"description"`
	Enclosure   Enclosure `xml:"enclosure"`
}

type Channel struct {
	Title       string    `xml:"title"`
	Link        string    `xml:"link"`
	Description string    `xml:"description"`
	Items       []RssItem `xml:"item"`
}

type Source interface {
	GetRSS(feedURL string) *Rss
	DetectContentType(data []byte) string
	Get(url string) (resp *http.Response, err error)
}

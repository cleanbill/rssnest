package feeds

import (
	"time"
)

type RssResult struct {
	Name        string
	Date        time.Time
	Item        RssItem
	Link        string
	AlreadyHave bool
	Failed      bool
	MessageSent bool
	FailReason  string
	Message     string
}

type Store interface {
	GetLast(i int, name string) []RssResult
	GetToMessage() []RssResult
	AlreadyHave(itemLink string, name string) bool
	Save(rssResult RssResult, link string, name string)
}

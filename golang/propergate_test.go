package main

import (
	"fmt"
	"github.com/bangbangsoftware/config"
	"github.com/bangbangsoftware/feeds"
	"testing"
)

var lastSentItems []feeds.RssResult
var lastMsg string
var lastUrl string
var last []feeds.RssResult
var messages []feeds.RssResult

// mock Store
type mockStore struct{}

func (ms mockStore) ClearMessages() {}
func (ms mockStore) GetLast(i int, filename string) []feeds.RssResult {
	return last
}
func (ms mockStore) GetToMessage() []feeds.RssResult {
	return messages
}
func (ms mockStore) AlreadyHave(itemLink string, filename string) bool {
	return false
}
func (ms mockStore) Save(rssResult feeds.RssResult, link string, filename string) {}

// mock target
type mockTarget struct{}

func (mt mockTarget) Send(newItems []feeds.RssResult, prices []feeds.GoldMoney) {
	lastSentItems = newItems
	if len(lastSentItems) == 0 {
		fmt.Println("Mock store Send was just called with empty array, should I error?")
		return
	}
	for i := range lastSentItems {
		var res = lastSentItems[i]
		var msg = res.Item.Title
		var url = res.Link
		fmt.Println(i, ". Mock store is storing ", msg, " with ", url, " link")
	}
}

func (mt mockTarget) Message(ignore Shortener, msg string, url string) {
	fmt.Println("mock store is messaging ", msg, " with ", url, " link")
	lastMsg = msg
	lastUrl = url
}

// mock prices
var mockPrices []feeds.GoldMoney

// mock shorterner
type mockShort struct{}

func (ms mockShort) Shorten(feedURL string, apiKey string) *ShortURL {
	return new(ShortURL)
}

// mock conf
var mockConf = new(config.Settings)

func TestSend(t *testing.T) {
	mStore := new(mockStore)
	mTarget := new(mockTarget)
	mShort := new(mockShort)
	mockConf.Propagate.QtyPerPage = 1

	var checkLastAreSent = func() {
		for i := range lastSentItems {
			if last[i] != lastSentItems[i] {
				t.Error("The ", i, " item in store was not sent")
			}
		}
	}

	var result feeds.RssResult
	result.Item.Title = "boom"
	result.Link = "http://www.boom.org"
	last = append(last, result)
	var result2 feeds.RssResult
	result2.Item.Title = "boomboom"
	result2.Link = "http://www.boomboom.org"
	last = append(last, result2)
	Propagate(mStore, mTarget, mockPrices, mShort, *mockConf)
	checkLastAreSent()

	mockConf.Propagate.QtyPerPage = 1000
	Propagate(mStore, mTarget, mockPrices, mShort, *mockConf)
	checkLastAreSent()
}

func TestMessage(t *testing.T) {
	mStore := new(mockStore)
	mTarget := new(mockTarget)
	mShort := new(mockShort)
	mockConf.Propagate.QtyPerPage = 1

	Propagate(mStore, mTarget, mockPrices, mShort, *mockConf)
	if lastMsg != "" || lastUrl != "" {
		t.Error("There should be no messages, but got:", lastMsg, " with this url:", lastUrl)
	}
	var result feeds.RssResult
	result.Item.Title = "boom"
	result.Link = "http://www.boom.org"
	messages = append(messages, result)

	Propagate(mStore, mTarget, mockPrices, mShort, *mockConf)
	if lastMsg != "boom" || lastUrl != result.Link {
		t.Error("There should be boom messages, but got:", lastMsg, " with this url:", lastUrl)
	}

	var result2 feeds.RssResult
	result2.Item.Title = "bangbang"
	result2.Link = "http://www.bangbang.org"
	messages = append(messages, result2)

	Propagate(mStore, mTarget, mockPrices, mShort, *mockConf)
	if lastMsg != "bangbang" || lastUrl != result2.Link {
		t.Error("There should be bangbang messages, but got:", lastMsg, " with this url:", lastUrl)
	}
}

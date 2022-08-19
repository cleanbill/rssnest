package config

import (
	"encoding/json"
	"io/ioutil"
	"log"
)

type GeneralConf struct {
	Feedfile  string
	AudioDir  string
	VisualDir string
	DataDir   string
	StoreName string
}

type FtpConf struct {
	Url  string
	Port int
	Usr  string
	Pw   string
}

type TweetConf struct {
	ConsumerKey       string
	ConsumerSecret    string
	AccessTokenKey    string
	AccessTokenSecret string
}

type PropagateConf struct {
	QtyPerPage int
	Ftp        FtpConf
	Tweet      TweetConf
	Apikey     string
}

type Settings struct {
	General   GeneralConf
	Propagate PropagateConf
}

var settings Settings

func GetConfig() Settings {
	return settings
}

func LoadConfig(path string) Settings {
	log.Printf("loading config from: %s \n", path)
	file, e := ioutil.ReadFile(path)
	if e != nil {
		log.Printf("Config File error: %v\n", e)
		return settings
	}
	json.Unmarshal(file, &settings)
	return settings
}

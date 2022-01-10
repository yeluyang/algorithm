package main

import (
	"github.com/sirupsen/logrus"
	"github.com/yeluyang/algorithm/playground/golang/pkg"
)

func main() {
	logrus.SetLevel(logrus.TraceLevel)
	logrus.Info("starting")
	pkg.Solution()
}

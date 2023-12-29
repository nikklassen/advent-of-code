//go:build js

package animate

import (
	"time"

	"honnef.co/go/js/dom/v2"
)

func Animate(window dom.Window, duration time.Duration, callback func(float64)) chan struct{} {
	done := make(chan struct{})
	var startTime time.Duration
	var inner func(time.Duration)
	inner = func(d time.Duration) {
		if startTime == 0 {
			startTime = d
		}
		elapsed := d - startTime
		if elapsed >= duration {
			elapsed = duration
		}
		complete := float64(elapsed) / float64(duration)
		callback(complete)
		if elapsed == duration {
			done <- struct{}{}
		} else {
			window.RequestAnimationFrame(inner)
		}
	}
	window.RequestAnimationFrame(inner)
	return done
}

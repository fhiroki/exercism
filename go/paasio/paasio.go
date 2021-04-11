package paasio

import (
	"io"
	"sync"
)

type counter struct {
	sync.Mutex
	fn    func([]byte) (int, error)
	bytes int64
	ops   int
}

func (c *counter) op(p []byte) (int, error) {
	n, err := c.fn(p)
	if err != nil {
		return 0, err
	}

	c.Lock()
	defer c.Unlock()
	c.bytes += int64(n)
	c.ops++
	return n, nil
}

func (c *counter) count() (int64, int) {
	c.Lock()
	defer c.Unlock()
	return c.bytes, c.ops
}

type readWriteCounter struct {
	rc *counter
	wc *counter
}

func (rw readWriteCounter) Write(p []byte) (int, error) {
	return rw.wc.op(p)
}

func (rw readWriteCounter) WriteCount() (int64, int) {
	return rw.wc.count()
}

func (rw readWriteCounter) Read(p []byte) (int, error) {
	return rw.rc.op(p)
}

func (rw readWriteCounter) ReadCount() (int64, int) {
	return rw.rc.count()
}

func NewWriteCounter(w io.Writer) WriteCounter {
	return &readWriteCounter{
		rc: nil,
		wc: &counter{fn: w.Write},
	}
}

func NewReadCounter(r io.Reader) ReadCounter {
	return &readWriteCounter{
		rc: &counter{fn: r.Read},
		wc: nil,
	}
}

func NewReadWriteCounter(rw io.ReadWriter) ReadWriteCounter {
	return &readWriteCounter{
		rc: &counter{fn: rw.Read},
		wc: &counter{fn: rw.Write},
	}
}

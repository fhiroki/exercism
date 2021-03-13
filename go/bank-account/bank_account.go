package account

import (
	"sync"
)

type Account struct {
	sync.Mutex
	balance  int
	isClosed bool
}

func Open(amt int) *Account {
	if amt < 0 {
		return nil
	}
	return &Account{balance: amt, isClosed: false}
}

func (a *Account) Close() (int, bool) {
	a.Lock()
	defer a.Unlock()
	if a.isClosed {
		return 0, false
	}
	a.isClosed = true
	return a.balance, true
}

func (a *Account) Balance() (int, bool) {
	a.Lock()
	defer a.Unlock()
	if a.isClosed {
		return 0, false
	}
	return a.balance, true
}

func (a *Account) Deposit(amt int) (int, bool) {
	a.Lock()
	defer a.Unlock()
	if a.isClosed || a.balance+amt < 0 {
		return 0, false
	}
	a.balance += amt
	return a.balance, true
}

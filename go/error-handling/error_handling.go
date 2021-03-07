package erratum

// Use handles various kinds of error.
func Use(o ResourceOpener, input string) (err error) {
	var res Resource
	for res, err = o(); err != nil; res, err = o() {
		if _, ok := err.(TransientError); !ok {
			return err
		}
	}

	defer res.Close()
	defer func() {
		if r := recover(); r != nil {
			switch e := r.(type) {
			case FrobError:
				res.Defrob(e.defrobTag)
				err = e
			case error:
				err = e
			}
		}
	}()

	res.Frob(input)
	return err
}

import test from 'ava'

import { pidPath } from '../index.js'

test('pidpath self', (t) => {
  t.is(
    pidPath(process.pid),
    process.execPath,
  )
})

test('pidpath 0', (t) => {
  pidPath(0)
})

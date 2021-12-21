import { portsList, Port } from './index'

const posts = portsList()

console.log(posts)

for (const i in posts) {
  let port: Port | null = null
  try {
    console.log('connection...', i)
    port = new Port(i)
  } catch (e) {
    console.error('connection error', e)
    continue
  }

  console.log('port', port)

  setInterval(() => {
    try {
      console.log('AT')
      port!.write('AT')
      console.log(port!.read())
    } catch (e) {
      console.error('connection read', e)
    }
  }, 1000)
}

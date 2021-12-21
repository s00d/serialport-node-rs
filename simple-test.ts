import { portsList, Port } from './index'


let posts = portsList();

console.log(posts)

for (let i in posts) {
  let port: Port|null = null;
  try {
    console.log("connection...", i)
    port = new Port(i);
  } catch (e) {
    console.log("connection error", e)
    continue;
  }

  console.log("port", port)

  setInterval(() => {
    try {
      console.log("AT")
      port!.write('AT')
      console.log(port!.read())
    } catch (e) {
      console.log("connection read", e)
    }
  }, 1000)

}

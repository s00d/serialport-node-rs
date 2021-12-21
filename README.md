# serialport-node-rs

> serialport project for writing node package with napi-rs and serialport-rc.

## Install this test package

```
npm i serialport-node-rs --save
```

## Example

```js
import { portsList, Port } from 'serialport-node-rs'

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
      console.log(port!.read()) // string buffer
    } catch (e) {
      console.log("connection read", e)
    }
  }, 1000)

}

```

## Release package

Ensure you have set you **NPM_TOKEN** in `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want release package:

```
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

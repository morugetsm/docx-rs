// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import("./pkg");
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`

rust
  .then(m => {
    m.greet("World!");
    let hoge = m.create(2);
    console.log(hoge);
    hoge.add();
    hoge.log();
  })
  .catch(console.error);

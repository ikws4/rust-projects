## Language Design Overview

I want the language syntax to be concise and familiar to developers with Java or C# backgrounds like myself, while being powerful enough for general use. The key features include:

- Dynamic typing, where types are used only for annotation and are not enforced by the interpreter.
- Uses a flat OOP design to avoid deep inheritance, where objects can only implement traits using the `:` operator
- `Trait` defines interface methods, while `object` encapsulates logic.
- Create anonymous objects with `{}` and arrays of objects using `[]`.

Here's an example of the language:

```
object RenderContext {
  init() {
    // Initialzie the context object ...
  }

  deinit() {
    // Release the resources ...
  }
}

trait Renderable {
  render(context: RenderContext): void;
}

trait Updatable {
  update(dt: number);
}

object Text : Renderable + Updatable {
  init(text) {
    this.text = text;
  }

  render(context) {
    // Rendering code ...
  }
}

object Circle : Renderable {
  init(position, radius) {
    this.position = position;
    this.text = text;
  }

  render(context) {
    // Rendering code ...
  }
}

var context = RenderContext {};
var renderables: Renderable = [
  Text {
    text = "Hello",
  },
  Circle {
    position = {
      x = 0,
      y = 0,
    },
    radius = 5,
  }
];

while true {
  for renderable in renderables {
    renderable.render(context);
  }
}
```

## Appendix

- The Antlr4 Grammar: [Juice.g4](./Juice.g4)

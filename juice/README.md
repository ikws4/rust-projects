## Language Design Overview

I want the language syntax to be concise and familiar to developers with Java or C# backgrounds like myself, while being powerful enough for general use. The key features include:

- Dynamic typing, where types are used only for annotation and are not enforced by the interpreter.
- Uses a flat OOP design to avoid deep inheritance, where objects can only implement traits using the `:` operator
- `Trait` defines interface methods, while `object` encapsulates logic.
- Create anonymous objects with `{}` and arrays of objects using `[]`.

Here's an example of the language:

```
object RenderContext {
  init() { }

  deinit() { }
}

trait Renderable {
  render(context: RenderContext): void;
}

trait Updatable {
  update(dt: number);
}

object Text : Renderable + Updatable {
  init(text) { }

  update(dt: number);

  render(context) {
    // Rendering code ...
    print("render text");
  }
}

object Circle : Renderable {
  init(position, radius) { }

  render(context) {
    // Rendering code ...
    print("render circle");
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

var frame = 0;
while (frame < 60) {
  for (var renderable in renderables) {
    renderable.render(context);
  }
  frame = frame + 1;
}
```

## Appendix

- The Antlr4 Grammar: [Juice.g4](./Juice/Juice.g4)

module.exports = class CreateProductController {
  constructor (useCase) {
    this._use_case = useCase
  }

  async handle (request, response) {
    const { name, description, src } = request.body

    const props = {
      name: {
        value: name
      },
      description: {
        description: description
      },
      image: {
        src: src
      }
    }

    const ctx = new Map(Object.entries(props))

    const res = await this._use_case.execute(ctx).then((value) => { // originalmente let
      return response.status(201).send(value)
    }).catch((e) => {
      return response.status(400).json({
        message: e.message || 'Unexpected error'
      })
    })
    return res // Objeto a rust
  }
}

/*     const obj = Object.assign({}, Object.create(this))
    Object.setPrototypeOf(obj, this)
    return obj */

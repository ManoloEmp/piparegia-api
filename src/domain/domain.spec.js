// const Product = require('./lib/domain').Product

/* class ProductRouter {
  route (httpRequest) {
    if (!httpRequest.body.name || !httpRequest.body.description) {
      return {
        statusCode: 400
      }
    }
  }
}

class HttpResponse {
  static badRequest () {
    return {

      message: (mes) => {
        return {
          mes: mes,
          statusCode: 400
        }
      }
    }
  }

  static serverError () {
    return {
      statusCode: 500
    }
  }

  static ok () {
    return {
      statusCode: 201
    }
  }
  static status (status) {
    return {

      send: (res) => {
        return {  // lo que recibe rust
          statusCode: status,
          response: res
        }
      },

      json: (object) => {
        return object
      }
    }
  }
}

class CreateProductController {
  constructor (createProductUC) {
    this.createProductUC = createProductUC
  }

  async handle (request, response) {
    const { name, description, src, alt } = request.body

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

    try {
      let res = await this.createProductUC.execute(ctx)
      console.log('respos', res)
      return response.status(201).send(res)
    } catch (e) {
      return response.status(400).json({
        message: e.message || 'Unexpected error'
      })
    }
  }
}

class CreateProductUC {
  constructor (repository) {
    this._repository = repository
  }
}

class PostgresProductRepo {

  constructor () {
    this.products = []
  }

  find_by_name (name) {
    const promise = new Promise((resolve, reject) => {
      const product = this.products.find(product => {
        console.log('product_name:', product.name.value.slice(1, -1), ' ', 'name:', name)
        return product.name.value.slice(1, -1) === name
      })
      if (product) {
        resolve(product.name.value.slice(1, -1))
      }
      resolve(product)
    })
    return promise
  }

  save (product) {
    const promise = new Promise((resolve, reject) => {
      this.products.push(product)
      resolve({
        first_name: this.products[0].name,
        total: this.products.length
      })
    })

    return promise
  }
}

const postgres = new PostgresProductRepo()

describe('test rust wasm_bindgen funtionalities', () => {
  // Applies only to tests in this describe block
  test('Instantiante product with constructor method', () => {
    const props = {
      name: {
        value: 'Peperoni'
      },
      description: {
        description: 'Pizza orgánica con peperoni'
      },
      image: {
        src: '/peperoni.png'
      }
    }

    const ctx = new Map(Object.entries(props))

    const product = new Product(ctx)
    let str = '\"'
    // str.concat('Peperoni', str)
    expect(product.name.value.slice(1, -1)).toBe('Peperoni')
    expect(product.image.src.slice(1, -1)).toBe('/peperoni.png')
  })

  test('Call multiple funtions with same prototipe', () => {
    const props = {
      name: {
        value: 'Peperoni'
      },
      description: {
        description: 'Pizza orgánica con peperoni'
      },
      image: {
        src: '/peperoni.png'
      }
    }

    const ctx = new Map(Object.entries(props))

    const product = new Product(ctx)
    let str = '\"'

    product.name = 'Tomates'

    // str.concat('Peperoni', str)
    expect(product.name.value).toBe('Tomates')
    product.name = 'Parmezano'
    expect(product.name.value).toBe('Parmezano')
    expect(product.image.src.slice(1, -1)).toBe('/peperoni.png')
  })

}) */

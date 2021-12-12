// const Product = require('./product.entitie')
const { Value, Product } = require('./product.entitie')
const { PrototypeError } = require('../errors/entities-errors.js')
const Root = require('../helpers/validator')

// const product = new Product()

describe('Entities validators', () => {
  test('Should pront PrototypeError if name object it´s not  Value prototype', () => {
    const sut = Value._create('image')

    expect(sut.is_proto(sut, Product)).toEqual(new PrototypeError('this', 'Value'))
  })

  test('Should receive a correct argument', () => {
    const type = 'name'
    const sut = Value._create(type, { value: 'peperoni' })

    expect(sut.get_definition(sut)).toEqual({ value: 'peperoni' })
  })

  test('Should set read only properties in Object creation', () => {
    const type = 'description'
    const sut = Value._create(type, {
      value: 'Pizza orgánica'
    })

    const newObj = new Root()

    const strSchema = newObj.extend(
      sut
    )

    const b = Object.getOwnPropertyDescriptor(strSchema._definition, 'value')

    expect(b).toEqual({
      configurable: false,
      enumerable: true,
      value: 'Pizza orgánica',
      writable: false
    })
  })
})

/* const Root = require ('./validator')
const { TypeParamError, MissingParamError, PrototypeError, ReadError} = require('../errors/entities-errors')
const { Value, Product} = require('../entities/product.entitie')

describe('Entities validators', () => {
  test('Should generate error if bad name is provided', () => {
    const args = {
      value: 'Peperoni'
    }

    // const sut = Root.create('name', args)

    const newObj = new Root()

    console.log('root: ', newObj)

    const type = 'description'

    const proto = Value._create(type, {
      value: 'Pizza orgánica'
    })

    const strSchema = newObj.extend(
      proto
    )

    expect(strSchema._rules.get_definition(strSchema)).toEqual({
      value: 'Pizza orgánica'
    })
  })

  test('Should generate error if bad name is provided', () => {
    const args = {
      value: 'Peperoni'
    }

    // const sut = Root.create('name', args)

    const newObj = new Root()

    console.log('root: ', newObj)

    const str = newObj.extend(
      {
        _type: 'string',
        typeof: 'primitive',
        requiered: function () {
          return true
        }

      }
    )

    const type = 'description'

    const proto = Value._create(type, {

      typeof: 'value_object',
      attributes: []
    })

    const desc = newObj.extend(
      proto
    )

    const sut = desc.description({
      value: str.string()
    })

    expect(sut).toEqual({
      value: ''
    })
  })
}) */

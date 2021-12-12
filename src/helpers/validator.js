/* const { TypeParamError, MissingParamError } = require('../errors/entities-errors')
const { Value, Product} = require('../entities/product.entitie')
const core = new Object()

core.Root = class {
  _definition = {}
  _rules = {}
  _type = Symbol('type')

  constructor(type, props) {

    this[this._type] = ''
    if (type && props.length > 0 && Object.entries(...props).length > 0 && Object.entries(this._definition).length === 0) {
      Object.assign(this._definition, ...props)
      this[this._type] = type
    }
    this.result = {
      error: {
        argument: {},
        details: []
      },
      value: {

      },
      message: []
    }

  }

  static create(type, ...schema) {
    return new core.Root(type, schema)
  }

  clone(instance) {
    console.log("proto es {}", Object.getPrototypeOf(instance))
    return Object.assign(
      Object.create(
        // Set the prototype of the new object to the prototype of the instance.
        // Used to allow new object behave like class instance.
        //Object.getPrototypeOf(instance),
        instance,
      ),
      // Prevent shallow copies of nested structures like arrays, etc
      JSON.parse(JSON.stringify(instance)),
    )

  }

  extend(options) {
    return core.extend(this, options)
  }

  _assign(target, type, definition) {

    //target.#type = this.#type
    console.log("defafter", definition);
    target[target._type] = type
    Object.assign(target._definition, definition)
    console.log("target def", target._definition);
    Object.freeze(target._definition)
   return target
  }

  type() {
    return this[this._type]
  }

  validate(options) {
    //if (Object.entries(this._definition).length === 0) {
    //  throw  'Should provide a schema'
    //}
    let schema = new Map(Object.entries(this._definition))
    schema.forEach((value, key, map) => {
      switch (typeof value) {
        case 'string':
            Object.assign(this.result, {
              ...this.result,
              value: {...this.result.value, [key]:value},
              message: [...this.result.message, `${key}: ${value} is a string`]
            })

            break;
        case 'number':
            Object.assign(this.result, {
              ...this.result,
              value: {...this.result.value, [key]:value},
              message: [...this.result.message, `${key}: ${value} is a number`]
            })
            break;
        case 'boolean':
            Object.assign(this.result, {
              ...this.result,
              value: {...this.result.value, [key]:value},
              message: [...this.result.message, `${key}: ${value} is boolean`]
            })
            break;
        case 'object':
            //console.log("error: ", value instanceof TypeParamError);
            if (value instanceof TypeParamError) {
              Object.assign(this.result.error, {
                ...this.result.error,
                details: [...this.result.error.details, value]
              })

            }

            break;
        case 'symbol':

            break;
        default:
          return 'unfefined'

      }
    })
    if (this.result.error) {

    }
    return this.result
  }

}

core.extend = function (from, options) {

  const def = Object.assign({}, options);

  console.log("defdef", def._definition);

  const proto = from.clone(from)
  console.log("proto val", Object.getPrototypeOf(options) === Value);

  //Object.setPrototypeOf(proto, Object.getPrototypeOf(options))

  const schema = from._assign(proto, def._type, def._definition)

  //const _proto = Object.setPrototypeOf(proto, core.Root)

  //proto._definition = def;
  //const parent = from._definition

  //schema.type = def.type;
  let _def = {}

  const properties = Object.assign({}, Object.getPrototypeOf(options));

  for (var prop in properties) {
    if (properties.hasOwnProperty(prop) && prop !== '_type' && prop !== '_definition') {
        _def = Object.assign({}, {[prop]: properties[prop]}, _def)

    }
  }

  console.log("rulrul", _def);

  Object.getPrototypeOf(schema).[schema[schema._type]] = function (schema) {
    if (schema) {
      return schema
    } else {
      return ''
    }
  }
  schema._rules = _def;

  //Object.freeze(schema._rules)

  //const rules = Object.assign({}, parent.rules);

  return schema
}

module.exports = core.Root */

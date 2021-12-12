class Variant {
  constructor (id, price, size, productId) {
    this._id = id
    this._price = price
    this._size = size
    this._product_id = productId
  }

  static create (id, price, size, productId) {
    return new Variant(id, price, size, productId)
  }
}

module.exports = {
  Variant
}

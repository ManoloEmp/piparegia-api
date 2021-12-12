module.exports = class MyClass {
  constructor () {
    this._number = 42
  }

  get number () {
    return this._number
  }

  set number (n) {
    return this._number
  }

  render () {
    return `My number is: ${this.number}`
  }
}

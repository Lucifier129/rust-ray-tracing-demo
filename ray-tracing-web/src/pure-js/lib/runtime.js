const { vec3 } = require('gl-matrix')

const isVec3 = obj => obj instanceof Float32Array
const toVec3 = n => vec3.fromValues(n, n, n)

let _add_ = function(left, right) {
  if (!isVec3(left) && !isVec3(right)) return left + right

  if (!isVec3(left)) left = toVec3(left)
  if (!isVec3(right)) right = toVec3(right)

  return vec3.add(vec3.create(), left, right)
}

let _sub_ = function(left, right) {
  if (!isVec3(left) && !isVec3(right)) return left - right

  if (!isVec3(left)) left = toVec3(left)
  if (!isVec3(right)) right = toVec3(right)

  return vec3.sub(vec3.create(), left, right)
}

let _mul_ = function(left, right) {
  if (!isVec3(left) && !isVec3(right)) return left * right

  if (!isVec3(left)) left = toVec3(left)
  if (!isVec3(right)) right = toVec3(right)

  return vec3.mul(vec3.create(), left, right)
}

let _div_ = function(left, right) {
  if (!isVec3(left) && !isVec3(right)) return left / right

  if (!isVec3(left)) left = toVec3(left)
  if (!isVec3(right)) right = toVec3(right)

  return vec3.div(vec3.create(), left, right)
}

let _negate_ = function(operand) {
  if (!isVec3(operand)) return -operand
  return vec3.negate(vec3.create(), operand)
}

module.exports = {
  _add_,
  _sub_,
  _mul_,
  _div_,
  _negate_
}

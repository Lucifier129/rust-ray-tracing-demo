const {
  vec3
} = require('gl-matrix');

const {
  _add_,
  _sub_,
  _mul_,
  _div_,
  _negate_
} = require('./lib/runtime');

const from = (x, y, z) => vec3.fromValues(x, y, z);

const dot = (vec3a, vec3b) => vec3.dot(vec3a, vec3b);

const cross = (vec3a, vec3b) => vec3.cross(vec3.create(), vec3a, vec3b);

const normalize = vec3a => vec3.normalize(vec3.create(), vec3a);

class Ray {
  constructor(a, b, time = 0.0) {
    this.a = a;
    this.b = b;
    this.time = time;
  }

  origin() {
    return this.a;
  }

  direction() {
    return this.b;
  }

  pointAt(t) {
    let {
      a,
      b
    } = this;
    return _add_(a, _mul_(t, b));
  }

} // linear-blend/linear-interpolation


const lerp = (t, start, end) => _add_(_mul_(_sub_(1.0, t), start), _mul_(t, end));

const createRecord = () => {
  return {
    t: 0,
    p: vec3.create(),
    normal: vec3.create()
  };
};

class Sphere {
  constructor(center, radius, material) {
    this.center = center;
    this.radius = radius;
    this.material = material;
  }

  setCenter(center) {
    this.center = center;
  }

  hit(ray, tmin, tmax, record) {
    let oc = _sub_(ray.origin(), this.center);

    let a = dot(ray.direction(), ray.direction());
    let b = dot(oc, ray.direction());

    let c = _sub_(dot(oc, oc), _mul_(this.radius, this.radius));

    let discriminant = _sub_(_mul_(b, b), _mul_(a, c));

    if (discriminant < 0) return false;

    let temp = _div_(_sub_(_negate_(b), Math.sqrt(discriminant)), a);

    let isValid = temp < tmax && temp > tmin;

    if (!isValid) {
      temp = _div_(_add_(_negate_(b), Math.sqrt(discriminant)), a);
      isValid = temp < tmax && temp > tmin;
    }

    if (isValid) {
      record.t = temp;
      record.p = ray.pointAt(temp);
      record.normal = _div_(_sub_(record.p, this.center), this.radius);
      record.material = this.material;
      return true;
    }

    return false;
  }

}

const getCenterByTime = (center1, center2, time1, time2, time) => {
  return _add_(center1, _mul_(_div_(_sub_(time, time1), _sub_(time2, time1)), _sub_(center2, center1)));
};

class MovingSphere {
  constructor(center1, center2, time1, time2, radius, material) {
    this.center1 = center1;
    this.center2 = center2;
    this.time1 = time1;
    this.time2 = time2;
    this.sphere = new Sphere(center1, radius, material);
  }

  hit(ray, tmin, tmax, record) {
    let center = getCenterByTime(this.center1, this.center2, this.time1, this.time2, ray.time);
    this.sphere.setCenter(center);
    return this.sphere.hit(ray, tmin, tmax, record);
  }

}

class HitableList {
  constructor(list) {
    this.list = list;
  }

  hit(ray, tmin, tmax, record) {
    let tempRecord = createRecord();
    let hited = false;
    let closest = tmax;

    for (let object of this.list) {
      if (object.hit(ray, tmin, closest, tempRecord)) {
        hited = true;
        closest = tempRecord.t;
        Object.assign(record, tempRecord);
      }
    }

    return hited;
  }

  push(...args) {
    this.list.push(...args);
  }

}

const randomInUnitDisk = () => {
  let p;

  do {
    let randomVec3 = from(Math.random(), Math.random(), 0.0);
    p = _sub_(_mul_(2.0, randomVec3), from(1.0, 1.0, 0.0));
  } while (dot(p, p) >= 1.0);

  return p;
};

class Camera {
  constructor(lookFrom, lookAt, vup, vfov, aspect, aperture, focusDist, time1, time2) {
    let theta = _div_(_mul_(vfov, Math.PI), 180);

    let halfHeight = Math.tan(_div_(theta, 2));

    let halfWidth = _mul_(halfHeight, aspect);

    let origin = lookFrom;
    let w = normalize(_sub_(lookFrom, lookAt));
    let u = normalize(cross(vup, w));
    let v = cross(w, u);

    let lowerLeftCorner = _sub_(origin, _mul_(focusDist, _add_(_add_(_mul_(u, halfWidth), _mul_(v, halfHeight)), w)));

    let horizontal = _mul_(_mul_(_mul_(focusDist, 2), u), halfWidth);

    let vertical = _mul_(_mul_(_mul_(focusDist, 2), v), halfHeight);

    this.time1 = time1;
    this.time2 = time2;
    this.u = u;
    this.v = v;
    this.origin = origin;
    this.lowerLeftCorner = lowerLeftCorner;
    this.horizontal = horizontal;
    this.vertical = vertical;
    this.lensRadius = _div_(aperture, 2);
  }

  getRay(s, t) {
    let {
      origin,
      lowerLeftCorner,
      horizontal,
      vertical
    } = this;

    let rd = _mul_(this.lensRadius, randomInUnitDisk());

    let offset = _add_(_mul_(this.u, rd[0]), _mul_(this.v, rd[1]));

    let direction = _sub_(_add_(_add_(lowerLeftCorner, _mul_(s, horizontal)), _mul_(t, vertical)), origin);

    let time = _add_(this.time1, _mul_(Math.random(), _sub_(this.time2, this.time1)));

    let ray = new Ray(_add_(origin, offset), _sub_(direction, offset), time);
    return ray;
  }

}

const randomInUnitSphere = () => {
  let p;

  do {
    let randomVec3 = from(Math.random(), Math.random(), Math.random());
    p = _sub_(_mul_(2.0, randomVec3), from(1.0, 1.0, 1.0));
  } while (vec3.squaredLength(p) >= 1.0);

  return p;
};

class LambertianMaterial {
  constructor(albedo) {
    this.albedo = albedo;
  }

  scatter(ray, record, ref) {
    let target = _add_(_add_(record.p, record.normal), randomInUnitSphere());

    let direction = _sub_(target, record.p);

    ref.scattered = new Ray(record.p, direction, ray.time);
    ref.attenuation = this.albedo;
    return true;
  }

}

const reflect = (v, n) => {
  return _sub_(v, _mul_(_mul_(2.0, dot(v, n)), n));
};

class MetalMaterial {
  constructor(albedo, fuzz = 1) {
    this.albedo = albedo;
    this.fuzz = Math.min(fuzz, 1);
  }

  scatter(ray, record, ref) {
    let reflected = reflect(vec3.normalize(vec3.create(), ray.direction()), record.normal);
    ref.scattered = new Ray(record.p, _add_(reflected, _mul_(this.fuzz, randomInUnitSphere())));
    ref.attenuation = this.albedo;
    return dot(ref.scattered.direction(), record.normal) > 0;
  }

}

const refract = (v, normal, ni_over_nt, ref) => {
  let uv = vec3.normalize(vec3.create(), v);
  let dt = dot(uv, normal);

  let discriminant = _sub_(1.0, _mul_(_mul_(ni_over_nt, ni_over_nt), _sub_(1.0, _mul_(dt, dt))));

  if (discriminant > 0) {
    ref.refracted = _sub_(_mul_(ni_over_nt, _sub_(uv, _mul_(normal, dt))), _mul_(normal, Math.sqrt(discriminant)));
    return true;
  }

  return false;
};

const schlick = (consine, refractIndex) => {
  let r0 = _div_(_sub_(1, refractIndex), _add_(1, refractIndex));

  r0 = _mul_(r0, r0);
  return _add_(r0, _mul_(_sub_(1, r0), Math.pow(_sub_(1, consine), 5)));
};

class DielectricMaterial {
  constructor(refractIndex) {
    this.refractIndex = refractIndex;
  }

  scatter(ray, record, ref) {
    let outwardNormal = vec3.create();
    let reflected = reflect(ray.direction(), record.normal);
    let ni_over_nt = 0.0;
    let consine = 0.0;
    ref.attenuation = from(1.0, 1.0, 1.0);
    let dotResult = dot(ray.direction(), record.normal);

    if (dotResult > 0) {
      outwardNormal = _negate_(record.normal);
      ni_over_nt = this.refractIndex;
      consine = _div_(_mul_(this.refractIndex, dotResult), vec3.length(ray.direction()));
    } else {
      outwardNormal = record.normal;
      ni_over_nt = _div_(1.0, this.refractIndex);
      consine = _div_(_negate_(dotResult), vec3.length(ray.direction()));
    }

    let refractProb = 0.0;

    if (refract(ray.direction(), outwardNormal, ni_over_nt, ref)) {
      refractProb = schlick(consine, this.refractIndex);
    } else {
      refractProb = 1.0;
    }

    if (Math.random() < refractProb) {
      ref.scattered = new Ray(record.p, reflected);
    } else {
      ref.scattered = new Ray(record.p, ref.refracted);
    }

    return true;
  }

}

const color = (ray, world, depth = 0) => {
  let record = createRecord();

  if (world.hit(ray, 0.001, Infinity, record)) {
    let ref = {
      scattered: null,
      attenuation: 0
    };

    if (depth >= 50 || !record.material.scatter(ray, record, ref)) {
      return from(0.0, 0.0, 0.0);
    }

    return _mul_(ref.attenuation, color(ref.scattered, world, _add_(depth, 1)));
  }

  let direction = vec3.normalize(vec3.create(), ray.direction());
  return lerp(_mul_(0.5, _add_(direction[1], 1.0)), from(1.0, 1.0, 1.0), from(0.5, 0.7, 1.0));
};

const randomScene = amount => {
  let list = [];
  let range = Math.floor(_div_(Math.sqrt(amount), 2));

  let start = _negate_(range);

  let end = +range;
  list[0] = new Sphere(from(0.0, -1000, 0), 1000, new LambertianMaterial(from(0.5, 0.5, 0.5)));
  let i = 1;
  let based = from(4.0, 0.2, 0.0);

  for (let a = start; a < end; a++) {
    for (let b = start; b < end; b++) {
      let chooseMat = Math.random();
      let center = from(_add_(a, _mul_(0.9, Math.random())), 0.2, _add_(b, _mul_(0.9, Math.random())));

      if (vec3.length(_sub_(center, based)) > 0.9) {
        if (chooseMat < 0.8) {
          let material = new LambertianMaterial(from(_mul_(Math.random(), Math.random()), _mul_(Math.random(), Math.random()), _mul_(Math.random(), Math.random()))); // list[i++] = new Sphere(center, 0.2, material)
          // continue

          list[i++] = new MovingSphere(center, _add_(center, from(0, _mul_(0.5, Math.random()), 0)), 0.0, 1.0, 0.2, material);
        } else if (chooseMat < 0.95) {
          let material = new MetalMaterial(from(_mul_(0.5, _add_(1, Math.random())), _mul_(0.5, _add_(1, Math.random())), _mul_(0.5, _add_(1, Math.random()))), _mul_(0.5, Math.random()));
          list[i++] = new Sphere(center, 0.2, material);
        } else {
          list[i++] = new Sphere(center, 0.2, new DielectricMaterial(1.5));
        }
      }
    }
  }

  list[i++] = new Sphere(from(0.0, 1.0, 0.0), 1.0, new DielectricMaterial(1.5));
  list[i++] = new Sphere(from(-4.0, 1.0, 0.0), 1.0, new LambertianMaterial(from(0.4, 0.2, 0.1)));
  list[i++] = new Sphere(from(4.0, 1.0, 0.0), 1.0, new MetalMaterial(from(0.7, 0.6, 0.5), 0.0));
  return new HitableList(list);
};

export default (({
  width = 800,
  height = 400,
  amount = 2,
  lookFrom = from(13.0, 2.0, 3.0),
  lookAt = from(0.0, 0.0, 0.0),
  vup = from(0.0, 1.0, 0.0),
  vfov = 20,
  aspect = _div_(width, height),
  focusDist = 10,
  aperture = 0.1
}) => {
  let nx = width;
  let ny = height;
  let camera = new Camera(lookFrom, lookAt, vup, vfov, aspect, aperture, focusDist, 0.0, 1.0);
  let world = randomScene(amount);

  let renderByPosition = (x, y) => {
    let u = _div_(_add_(x, Math.random()), nx);

    let v = _div_(_add_(y, Math.random()), ny);

    let ray = camera.getRay(u, v);
    return color(ray, world);
  };

  let render = function* () {
    for (let j = _sub_(ny, 1); j >= 0; j--) {
      for (let i = 0; i < nx; i++) {
        let [r, g, b] = renderByPosition(i, j);
        let a = 1;
        yield r;
        yield g;
        yield b;
        yield a;
      }
    }
  };

  return {
    render,
    renderByPosition
  };
});
const fs = require('fs')

let data = fs.readFileSync('core.asset.json')

let json = JSON.parse(data)

let attr_id = 0
for (let attr of json.attributes) {
    attr.id = attr_id++
}

let cate_id = 0
let lot_id = 0
for (let cate of json.categories) {
    cate.id = cate_id++

    if (cate.lots === undefined) {
        continue
    }

    for (let lot of cate.lots) {
        lot.id = lot_id++
    }
}

let output = JSON.stringify(json)

fs.writeFileSync('core.asset.json', output)
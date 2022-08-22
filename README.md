# mock-server

`mock-server` is a json server that you can create mock data like `mockjs`

## How to use
```
mock-server --config config.json --port 8080
```

## config.json 

```json
{

  // config part
  "config": {

    // routing settings
    "routing": {
      // routing path
      // through /a/b/c/1 to access /api/<data>/1
      "/a/b/c/:id": { 
        "to": "/api/data/:id",
      }
    },

    // result wrapping
    "wrapping": {

      // success return wrap
      "ok": {
        "code": 200,

        // message placeholder
        "msg": "$msg", 

        // data placeholder
        "data": "$data"
      },

      // error wrap
      "err": {
        "code": 500,
        "msg": "$msg",
        "data": "$data"
      },

      // pagination wrap
      // $page, $size, $total, $items is the placeholder to replace
      "pagination": {
        "page": "$page",
        "size": "$size",
        "total": "$total",
        "items": "$items"
      }

    }
  },

  // data part 
  "data": {

      // data1 is the collection name
      // 100 is the  collection count
      // return data by array list
      // url: /api/data1
      "data1|100": [
      {

        // increasement id
        "id|+1": 1,

        // random uuid
        "i2": "@uuid",

        // return the item by order 
        "ser|+1": [ "a", "bb", "c" ],

        // repeat the value by the given count
        "start|3": "a",

        // just randomly pick one item
        "pick|1": ["a", "b", "c"],

        // return float data
        "float|.1-10": 0.5,
        "float2|10-100.3-10": 100.5,

        // return radom name
        "name": "@name",

        // return radom word by given range
        "word": "@word(5)",
        "word2": "@word(1, 10)",

        // return radom wsentence by given range
        "sentence": "@sentence",
        "sentence2": "@sentence(1, 10)",

        // return radom paragraph by given range
        "paragraph": "@paragraph",
        "paragraph2": "@paragraph(1, 10)"
      }
    ],

    // return data by object
    // url: /api/data2
    "data2" : {
      "id": "@uuid"
    },

    // return data by only a value
    // url: /api/data3
    "data3": "@name"

  }
}

```

## Query parameters

param|description| example
----|----|----
_q|  query |  /api/data1?_q=test
_page| page index, start from 1 | /api/data1?_page=1
_size| page size, default by 10 | /api/data1?_page=1&_size=5
_sort| sort key | /api/data1?_sort=name
_order| sort order by desc or asc | /api/data1?_sort=name&_order=desc





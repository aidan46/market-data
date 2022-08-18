# market-data
Pull market data from binance

# General information
- All endpoint return JSON object or array.
- Data is returned in ascending order. Oldest first, newest last.
- All time and time stamp related fields are in milliseconds.

## Endpoints
- For `GET` endpoints, parameters must be sent as a `query string`.
- For `POST`, `PUT`, and `DELETE` endpoints, the parameters may be sent a `query string` or in the `request body` with content type `application/x-www-form-urlencoded`. You may mix parameters between both the `query string` and `request body` if you wish to do so.
- Parameters may be sent in any order.
- If a parameter sent in both the `query string` and the `request body`, the `query string` parameter will be used.

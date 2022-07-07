const {GetBookRequest} = require('./bookstore_pb.js')
const {BookstorePromiseClient} = require('./bookstore_grpc_web_pb.js')

const client = new BookstorePromiseClient('http://localhost:5050')

const request = new GetBookRequest()
request.setId('World')

client.getBook(request).then(res => {
    console.log(res.getId())
}).catch(err => {
    console.log(err)
})

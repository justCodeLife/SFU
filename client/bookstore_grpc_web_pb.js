/**
 * @fileoverview gRPC-Web generated client stub for bookstore
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.bookstore = require('./bookstore_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.bookstore.BookstoreClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.bookstore.BookstorePromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.bookstore.GetBookRequest,
 *   !proto.bookstore.GetBookResponse>}
 */
const methodDescriptor_Bookstore_GetBook = new grpc.web.MethodDescriptor(
  '/bookstore.Bookstore/GetBook',
  grpc.web.MethodType.UNARY,
  proto.bookstore.GetBookRequest,
  proto.bookstore.GetBookResponse,
  /**
   * @param {!proto.bookstore.GetBookRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.bookstore.GetBookResponse.deserializeBinary
);


/**
 * @param {!proto.bookstore.GetBookRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.bookstore.GetBookResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.bookstore.GetBookResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.bookstore.BookstoreClient.prototype.getBook =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/bookstore.Bookstore/GetBook',
      request,
      metadata || {},
      methodDescriptor_Bookstore_GetBook,
      callback);
};


/**
 * @param {!proto.bookstore.GetBookRequest} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.bookstore.GetBookResponse>}
 *     Promise that resolves to the response
 */
proto.bookstore.BookstorePromiseClient.prototype.getBook =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/bookstore.Bookstore/GetBook',
      request,
      metadata || {},
      methodDescriptor_Bookstore_GetBook);
};


module.exports = proto.bookstore;


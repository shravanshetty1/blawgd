"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
exports.__esModule = true;
exports.MsgCreatePost = exports.Post = exports.protobufPackage = void 0;
/* eslint-disable */
var Long = __importStar(require("long"));
var minimal_1 = require("protobufjs/minimal");
exports.protobufPackage = "shravanshetty1.samachar.samachar";
var basePost = {
    creator: "",
    id: "",
    content: "",
    parentPost: "",
    blockNo: 0
};
exports.Post = {
    encode: function (message, writer) {
        if (writer === void 0) { writer = minimal_1.Writer.create(); }
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.id !== "") {
            writer.uint32(18).string(message.id);
        }
        if (message.content !== "") {
            writer.uint32(26).string(message.content);
        }
        if (message.parentPost !== "") {
            writer.uint32(34).string(message.parentPost);
        }
        if (message.blockNo !== 0) {
            writer.uint32(40).int64(message.blockNo);
        }
        return writer;
    },
    decode: function (input, length) {
        var reader = input instanceof Uint8Array ? new minimal_1.Reader(input) : input;
        var end = length === undefined ? reader.len : reader.pos + length;
        var message = __assign({}, basePost);
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.id = reader.string();
                    break;
                case 3:
                    message.content = reader.string();
                    break;
                case 4:
                    message.parentPost = reader.string();
                    break;
                case 5:
                    message.blockNo = longToNumber(reader.int64());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON: function (object) {
        var message = __assign({}, basePost);
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.id !== undefined && object.id !== null) {
            message.id = String(object.id);
        }
        else {
            message.id = "";
        }
        if (object.content !== undefined && object.content !== null) {
            message.content = String(object.content);
        }
        else {
            message.content = "";
        }
        if (object.parentPost !== undefined && object.parentPost !== null) {
            message.parentPost = String(object.parentPost);
        }
        else {
            message.parentPost = "";
        }
        if (object.blockNo !== undefined && object.blockNo !== null) {
            message.blockNo = Number(object.blockNo);
        }
        else {
            message.blockNo = 0;
        }
        return message;
    },
    toJSON: function (message) {
        var obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.id !== undefined && (obj.id = message.id);
        message.content !== undefined && (obj.content = message.content);
        message.parentPost !== undefined && (obj.parentPost = message.parentPost);
        message.blockNo !== undefined && (obj.blockNo = message.blockNo);
        return obj;
    },
    fromPartial: function (object) {
        var message = __assign({}, basePost);
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.id !== undefined && object.id !== null) {
            message.id = object.id;
        }
        else {
            message.id = "";
        }
        if (object.content !== undefined && object.content !== null) {
            message.content = object.content;
        }
        else {
            message.content = "";
        }
        if (object.parentPost !== undefined && object.parentPost !== null) {
            message.parentPost = object.parentPost;
        }
        else {
            message.parentPost = "";
        }
        if (object.blockNo !== undefined && object.blockNo !== null) {
            message.blockNo = object.blockNo;
        }
        else {
            message.blockNo = 0;
        }
        return message;
    }
};
var baseMsgCreatePost = {
    creator: "",
    content: "",
    parentPost: "",
    id: ""
};
exports.MsgCreatePost = {
    encode: function (message, writer) {
        if (writer === void 0) { writer = minimal_1.Writer.create(); }
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.content !== "") {
            writer.uint32(18).string(message.content);
        }
        if (message.parentPost !== "") {
            writer.uint32(26).string(message.parentPost);
        }
        if (message.id !== "") {
            writer.uint32(34).string(message.id);
        }
        return writer;
    },
    decode: function (input, length) {
        var reader = input instanceof Uint8Array ? new minimal_1.Reader(input) : input;
        var end = length === undefined ? reader.len : reader.pos + length;
        var message = __assign({}, baseMsgCreatePost);
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.content = reader.string();
                    break;
                case 3:
                    message.parentPost = reader.string();
                    break;
                case 4:
                    message.id = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON: function (object) {
        var message = __assign({}, baseMsgCreatePost);
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.content !== undefined && object.content !== null) {
            message.content = String(object.content);
        }
        else {
            message.content = "";
        }
        if (object.parentPost !== undefined && object.parentPost !== null) {
            message.parentPost = String(object.parentPost);
        }
        else {
            message.parentPost = "";
        }
        if (object.id !== undefined && object.id !== null) {
            message.id = String(object.id);
        }
        else {
            message.id = "";
        }
        return message;
    },
    toJSON: function (message) {
        var obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.content !== undefined && (obj.content = message.content);
        message.parentPost !== undefined && (obj.parentPost = message.parentPost);
        message.id !== undefined && (obj.id = message.id);
        return obj;
    },
    fromPartial: function (object) {
        var message = __assign({}, baseMsgCreatePost);
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.content !== undefined && object.content !== null) {
            message.content = object.content;
        }
        else {
            message.content = "";
        }
        if (object.parentPost !== undefined && object.parentPost !== null) {
            message.parentPost = object.parentPost;
        }
        else {
            message.parentPost = "";
        }
        if (object.id !== undefined && object.id !== null) {
            message.id = object.id;
        }
        else {
            message.id = "";
        }
        return message;
    }
};
var globalThis = (function () {
    if (typeof globalThis !== "undefined")
        return globalThis;
    if (typeof self !== "undefined")
        return self;
    if (typeof window !== "undefined")
        return window;
    if (typeof global !== "undefined")
        return global;
    throw "Unable to locate global object";
})();
function longToNumber(long) {
    if (long.gt(Number.MAX_SAFE_INTEGER)) {
        throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
    }
    return long.toNumber();
}
// @ts-ignore
if (minimal_1.util.Long !== Long) {
    minimal_1.util.Long = Long;
    minimal_1.configure();
}

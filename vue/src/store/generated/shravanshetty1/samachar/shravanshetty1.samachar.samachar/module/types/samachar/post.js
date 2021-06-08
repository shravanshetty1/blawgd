/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "shravanshetty1.samachar.samachar";
const basePost = {
    creator: "",
    id: "",
    content: "",
    parentPost: "",
    blockNo: 0,
};
export const Post = {
    encode(message, writer = Writer.create()) {
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
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...basePost };
        while (reader.pos < end) {
            const tag = reader.uint32();
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
    fromJSON(object) {
        const message = { ...basePost };
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
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.id !== undefined && (obj.id = message.id);
        message.content !== undefined && (obj.content = message.content);
        message.parentPost !== undefined && (obj.parentPost = message.parentPost);
        message.blockNo !== undefined && (obj.blockNo = message.blockNo);
        return obj;
    },
    fromPartial(object) {
        const message = { ...basePost };
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
    },
};
const baseMsgCreatePost = {
    creator: "",
    content: "",
    parentPost: "",
    id: "",
};
export const MsgCreatePost = {
    encode(message, writer = Writer.create()) {
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
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgCreatePost };
        while (reader.pos < end) {
            const tag = reader.uint32();
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
    fromJSON(object) {
        const message = { ...baseMsgCreatePost };
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
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.content !== undefined && (obj.content = message.content);
        message.parentPost !== undefined && (obj.parentPost = message.parentPost);
        message.id !== undefined && (obj.id = message.id);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgCreatePost };
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
    },
};
var globalThis = (() => {
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
if (util.Long !== Long) {
    util.Long = Long;
    configure();
}

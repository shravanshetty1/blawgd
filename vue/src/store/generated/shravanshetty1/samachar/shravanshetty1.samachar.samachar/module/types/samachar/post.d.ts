import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "shravanshetty1.samachar.samachar";
/** proto/blog/post.proto */
export interface Post {
    creator: string;
    id: string;
    content: string;
    parentPost: string;
    blockNo: number;
}
export interface MsgCreatePost {
    creator: string;
    content: string;
    parentPost: string;
    id: string;
}
export declare const Post: {
    encode(message: Post, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): Post;
    fromJSON(object: any): Post;
    toJSON(message: Post): unknown;
    fromPartial(object: DeepPartial<Post>): Post;
};
export declare const MsgCreatePost: {
    encode(message: MsgCreatePost, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePost;
    fromJSON(object: any): MsgCreatePost;
    toJSON(message: MsgCreatePost): unknown;
    fromPartial(object: DeepPartial<MsgCreatePost>): MsgCreatePost;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};

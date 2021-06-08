export declare const protobufPackage = "shravanshetty1.samachar.samachar";
/** Query defines the gRPC querier service. */
export interface Query {
}
export declare class QueryClientImpl implements Query {
    private readonly rpc;
    constructor(rpc: Rpc);
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
export {};

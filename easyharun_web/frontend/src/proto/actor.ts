/* eslint-disable */
import { grpc } from "@improbable-eng/grpc-web";
import { BrowserHeaders } from "browser-headers";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "proto_actor";

export interface PingRequest {
  id: string;
}

export interface PingResponse {
  id: string;
}

export interface ActorsRunningGetRequest {
}

export interface ActorsRunningGetResponseItem {
  actorId: string;
  actorName: string;
  actorType: string;
}

export interface ActorsRunningGetResponse {
  items: ActorsRunningGetResponseItem[];
}

function createBasePingRequest(): PingRequest {
  return { id: "" };
}

export const PingRequest = {
  encode(message: PingRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PingRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePingRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.id = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PingRequest {
    return { id: isSet(object.id) ? String(object.id) : "" };
  },

  toJSON(message: PingRequest): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  create<I extends Exact<DeepPartial<PingRequest>, I>>(base?: I): PingRequest {
    return PingRequest.fromPartial(base ?? {});
  },

  fromPartial<I extends Exact<DeepPartial<PingRequest>, I>>(object: I): PingRequest {
    const message = createBasePingRequest();
    message.id = object.id ?? "";
    return message;
  },
};

function createBasePingResponse(): PingResponse {
  return { id: "" };
}

export const PingResponse = {
  encode(message: PingResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PingResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePingResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.id = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PingResponse {
    return { id: isSet(object.id) ? String(object.id) : "" };
  },

  toJSON(message: PingResponse): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  create<I extends Exact<DeepPartial<PingResponse>, I>>(base?: I): PingResponse {
    return PingResponse.fromPartial(base ?? {});
  },

  fromPartial<I extends Exact<DeepPartial<PingResponse>, I>>(object: I): PingResponse {
    const message = createBasePingResponse();
    message.id = object.id ?? "";
    return message;
  },
};

function createBaseActorsRunningGetRequest(): ActorsRunningGetRequest {
  return {};
}

export const ActorsRunningGetRequest = {
  encode(_: ActorsRunningGetRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ActorsRunningGetRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseActorsRunningGetRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): ActorsRunningGetRequest {
    return {};
  },

  toJSON(_: ActorsRunningGetRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<ActorsRunningGetRequest>, I>>(base?: I): ActorsRunningGetRequest {
    return ActorsRunningGetRequest.fromPartial(base ?? {});
  },

  fromPartial<I extends Exact<DeepPartial<ActorsRunningGetRequest>, I>>(_: I): ActorsRunningGetRequest {
    const message = createBaseActorsRunningGetRequest();
    return message;
  },
};

function createBaseActorsRunningGetResponseItem(): ActorsRunningGetResponseItem {
  return { actorId: "", actorName: "", actorType: "" };
}

export const ActorsRunningGetResponseItem = {
  encode(message: ActorsRunningGetResponseItem, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.actorId !== "") {
      writer.uint32(10).string(message.actorId);
    }
    if (message.actorName !== "") {
      writer.uint32(18).string(message.actorName);
    }
    if (message.actorType !== "") {
      writer.uint32(26).string(message.actorType);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ActorsRunningGetResponseItem {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseActorsRunningGetResponseItem();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.actorId = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.actorName = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.actorType = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ActorsRunningGetResponseItem {
    return {
      actorId: isSet(object.actorId) ? String(object.actorId) : "",
      actorName: isSet(object.actorName) ? String(object.actorName) : "",
      actorType: isSet(object.actorType) ? String(object.actorType) : "",
    };
  },

  toJSON(message: ActorsRunningGetResponseItem): unknown {
    const obj: any = {};
    message.actorId !== undefined && (obj.actorId = message.actorId);
    message.actorName !== undefined && (obj.actorName = message.actorName);
    message.actorType !== undefined && (obj.actorType = message.actorType);
    return obj;
  },

  create<I extends Exact<DeepPartial<ActorsRunningGetResponseItem>, I>>(base?: I): ActorsRunningGetResponseItem {
    return ActorsRunningGetResponseItem.fromPartial(base ?? {});
  },

  fromPartial<I extends Exact<DeepPartial<ActorsRunningGetResponseItem>, I>>(object: I): ActorsRunningGetResponseItem {
    const message = createBaseActorsRunningGetResponseItem();
    message.actorId = object.actorId ?? "";
    message.actorName = object.actorName ?? "";
    message.actorType = object.actorType ?? "";
    return message;
  },
};

function createBaseActorsRunningGetResponse(): ActorsRunningGetResponse {
  return { items: [] };
}

export const ActorsRunningGetResponse = {
  encode(message: ActorsRunningGetResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.items) {
      ActorsRunningGetResponseItem.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ActorsRunningGetResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseActorsRunningGetResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.items.push(ActorsRunningGetResponseItem.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ActorsRunningGetResponse {
    return {
      items: Array.isArray(object?.items) ? object.items.map((e: any) => ActorsRunningGetResponseItem.fromJSON(e)) : [],
    };
  },

  toJSON(message: ActorsRunningGetResponse): unknown {
    const obj: any = {};
    if (message.items) {
      obj.items = message.items.map((e) => e ? ActorsRunningGetResponseItem.toJSON(e) : undefined);
    } else {
      obj.items = [];
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ActorsRunningGetResponse>, I>>(base?: I): ActorsRunningGetResponse {
    return ActorsRunningGetResponse.fromPartial(base ?? {});
  },

  fromPartial<I extends Exact<DeepPartial<ActorsRunningGetResponse>, I>>(object: I): ActorsRunningGetResponse {
    const message = createBaseActorsRunningGetResponse();
    message.items = object.items?.map((e) => ActorsRunningGetResponseItem.fromPartial(e)) || [];
    return message;
  },
};

export interface ActorService {
  /** Our SayHello rpc accepts HelloRequests and returns HelloReplies */
  ping(request: DeepPartial<PingRequest>, metadata?: grpc.Metadata): Promise<PingResponse>;
  actors_running_get(
    request: DeepPartial<ActorsRunningGetRequest>,
    metadata?: grpc.Metadata,
  ): Promise<ActorsRunningGetResponse>;
}

export class ActorServiceClientImpl implements ActorService {
  private readonly rpc: Rpc;

  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.ping = this.ping.bind(this);
    this.actors_running_get = this.actors_running_get.bind(this);
  }

  ping(request: DeepPartial<PingRequest>, metadata?: grpc.Metadata): Promise<PingResponse> {
    return this.rpc.unary(ActorServicepingDesc, PingRequest.fromPartial(request), metadata);
  }

  actors_running_get(
    request: DeepPartial<ActorsRunningGetRequest>,
    metadata?: grpc.Metadata,
  ): Promise<ActorsRunningGetResponse> {
    return this.rpc.unary(ActorServiceactors_running_getDesc, ActorsRunningGetRequest.fromPartial(request), metadata);
  }
}

export const ActorServiceDesc = { serviceName: "proto_actor.ActorService" };

export const ActorServicepingDesc: UnaryMethodDefinitionish = {
  methodName: "ping",
  service: ActorServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return PingRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = PingResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const ActorServiceactors_running_getDesc: UnaryMethodDefinitionish = {
  methodName: "actors_running_get",
  service: ActorServiceDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return ActorsRunningGetRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = ActorsRunningGetResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

interface UnaryMethodDefinitionishR extends grpc.UnaryMethodDefinition<any, any> {
  requestStream: any;
  responseStream: any;
}

type UnaryMethodDefinitionish = UnaryMethodDefinitionishR;

interface Rpc {
  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any>;
}

export class GrpcWebImpl {
  private host: string;
  private options: {
    transport?: grpc.TransportFactory;

    debug?: boolean;
    metadata?: grpc.Metadata;
    upStreamRetryCodes?: number[];
  };

  constructor(
    host: string,
    options: {
      transport?: grpc.TransportFactory;

      debug?: boolean;
      metadata?: grpc.Metadata;
      upStreamRetryCodes?: number[];
    },
  ) {
    this.host = host;
    this.options = options;
  }

  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    _request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any> {
    const request = { ..._request, ...methodDesc.requestType };
    const maybeCombinedMetadata = metadata && this.options.metadata
      ? new BrowserHeaders({ ...this.options?.metadata.headersMap, ...metadata?.headersMap })
      : metadata ?? this.options.metadata;
    return new Promise((resolve, reject) => {
      grpc.unary(methodDesc, {
        request,
        host: this.host,
        metadata: maybeCombinedMetadata ?? {},
        ...(this.options.transport !== undefined ? { transport: this.options.transport } : {}),
        debug: this.options.debug ?? false,
        onEnd: function (response) {
          if (response.status === grpc.Code.OK) {
            resolve(response.message!.toObject());
          } else {
            const err = new GrpcWebError(response.statusMessage, response.status, response.trailers);
            reject(err);
          }
        },
      });
    });
  }
}

declare const self: any | undefined;
declare const window: any | undefined;
declare const global: any | undefined;
const tsProtoGlobalThis: any = (() => {
  if (typeof globalThis !== "undefined") {
    return globalThis;
  }
  if (typeof self !== "undefined") {
    return self;
  }
  if (typeof window !== "undefined") {
    return window;
  }
  if (typeof global !== "undefined") {
    return global;
  }
  throw "Unable to locate global object";
})();

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

export class GrpcWebError extends tsProtoGlobalThis.Error {
  constructor(message: string, public code: grpc.Code, public metadata: grpc.Metadata) {
    super(message);
  }
}

import { grpc } from '@improbable-eng/grpc-web';

import {ActorServiceClientImpl, GrpcWebImpl} from "./proto/actor.ts";

const rpc = new GrpcWebImpl('http://localhost:50051', {
    // Only necessary for tests running on node. Remove the
    // transport config when actually using in the browser.
    transport: grpc.CrossBrowserHttpTransport({}),
    debug: false,
    metadata: new grpc.Metadata({ SomeHeader: 'bar' }),
});

export const GrpcClient = new ActorServiceClientImpl(rpc);
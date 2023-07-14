import {useEffect, useState} from "react";
import {GrpcClient} from "../grpc.ts";
import {ActorsRunningGetResponseItem} from "../proto/actor.ts";
import {Table} from "antd";

export function PageActors() {

    let [state, setState] = useState<{loading: boolean, actors: undefined|Array<ActorsRunningGetResponseItem>}>({
        loading: true,
        actors: undefined,
    });

    let reload = async () => {
        let actors = await GrpcClient.actors_running_get({});
        setState({
            loading: false,
            actors: actors.items
        })
    }

    useEffect(() => {
        reload();
    }, [])

    const columns = [
        {
            title: 'Actor Id',
            dataIndex: 'actorId',
            key: 'actorId',
        },
        {
            title: 'Name',
            dataIndex: 'actorName',
            key: 'actorName',
        },
        {
            title: 'Type',
            dataIndex: 'actorType',
            key: 'actorType',
        },
    ];

    const dataSource = state.actors?.map((v) => ({
        key: v.actorId,
        ...v
    }))

    return (
        <div>
            <Table dataSource={dataSource} columns={columns} />;
        </div>
    )
}
import React from 'react'
import Layout from "../../components/layout-admin";
import Button from "../../components/button";
import DeleteBtn from "../../components/delete-btn";
import TablePaginate from "../../components/table-paginate";
import { Dialog } from "@headlessui/react";
import { formatBoolean, formatArray,API, API_VER } from '../../utils';
import { useRouter } from 'next/router'
import { Switch } from '@headlessui/react'
import { useState, useEffect } from 'react';

export default function BuildPage() {
    const router = useRouter()
    const url = `${API}/${API_VER}/build`
    const [data, setData] = useState([]);

    let toggleActivation = async function (enabled, setEnabled, index, data) {
        let active = data[index].active ? false : true
        const response = await fetch(`${url}/active`, {
            method: "PUT",
            headers: {
              "Content-type": "application/json; charset=UTF-8",
              'Authorization': 'Bearer ' + localStorage.getItem("jwt")
            },
            body: JSON.stringify({id: data[index].id, active: active}),
        })

        if (response.ok) {
            let response_data = await response.json()
            data[index].active = response_data.active
            setEnabled(response_data.active)
            setData(data)
        }
    }

    let del = async function (index, data) {
        const response = await fetch(`${url}/${data[index].id}`, {
            method: "DELETE",
        });

        if (response.ok) {
            let data_copy = [...data];
            data_copy.splice(index, 1)
            setData(data_copy);

            // data.splice(index, 1) // update table
            // router.push("/admin/build", undefined, {shallow: true}) // force refresh of internal data
        }
    }

    let columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'Package', accessor: 'package',},
        { Header: 'Upstream Version', accessor: 'upstream_version',},
        { Header: 'Revision', accessor: 'revision',},
        { Header: 'Architectures', accessor: row => formatArray(row.architectures),},
        { Header: 'Firmware', accessor: 'firmware',},
        { Header: 'Publisher', accessor: 'publisher',},
        { Header: 'Insert Date', accessor: 'insert_date',},
        { Header: 'Active', accessor: 'active',
        Cell: (props) => {
            const [enabled, setEnabled] = useState(props.value)
            return (
                <Switch
                    checked={enabled}
                    onChange={() => toggleActivation(enabled, setEnabled, props.row.index, props.data)}
                    className={`${
                        enabled ? 'bg-blue-600' : 'bg-gray-200 dark:bg-gray-700'
                    } relative inline-flex items-center h-6 rounded-full w-11`}
                    >
                    <span className="sr-only">Toggle Activation</span>
                    <span
                        className={`${
                        enabled ? 'translate-x-6' : 'translate-x-1'
                        } inline-block w-4 h-4 transform bg-white rounded-full`}
                    />
                </Switch>
            )}
        },
        {
            Header: "Actions",
            accessor: "actions",
            Cell: (props) => {
                return (
                    <div>
                        <span onClick={() => del(props.row.index, props.data)}>
                            <DeleteBtn></DeleteBtn>
                        </span>
                    </div>
                );
            },
        }
    ];

    return (
        <Layout>
            <h1>Build</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
        </Layout>
    );
}

import Layout from "../../components/layout";
import Button from "../../components/button";
import DeleteBtn from "../../components/delete-btn";
import TablePaginate from "../../components/table-paginate";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatBoolean } from '../../utils';
import { postJsonForm } from "../../utils";
import { useRouter } from 'next/router'

export default function VersionPage() {
    const url = `http://127.0.0.1:8080/api/version`
    const router = useRouter()
    const [data, setData] = useState([]);

    let del = async function (row, data) {
        const response = await fetch(`${url}/${row.values.id}`, {
            method: "DELETE",
        });
        if (response.ok) {
            data.splice(row.index, 1) // update table
            router.push("/admin/version", undefined, {shallow: true}) // force refresh of internal data
        }
    }

    const columns = [
        { Header: 'ID', accessor: 'id' },
        { Header: 'Package', accessor: 'package' },
        { Header: 'Upstream Version', accessor: 'upstream_version' },
        { Header: 'Revision', accessor: 'revision' },
        { Header: 'Beta', accessor: 'beta' },
        // { Header: 'Services', accessor: 'services' },
        { Header: 'Insert Date', accessor: 'insert_date' },
        { Header: 'All Builds Active', accessor: row => formatBoolean(row.all_builds_active) },
        { Header: 'Install Wizard', accessor: row => formatBoolean(row.install_wizard) },
        { Header: 'Upgrade Wizard', accessor: row => formatBoolean(row.upgrade_wizard) },
        { Header: 'Startable', accessor: row => formatBoolean(row.startable) },
        {
            Header: "Actions",
            accessor: "actions",
            Cell: (props) => {
                const row = props.row;
                return (
                    <div>
                        {/* <span onClick={() => edit(row)}>
                            <i className="far fa-edit action mr-2">Edit</i>
                        </span> */}
                        <span onClick={i => del(row, props.data)}>
                            <DeleteBtn></DeleteBtn>
                        </span>
                    </div>
                );
            },
        }
    ];

    return (
        <Layout>
            <h1>Version</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
        </Layout>
    );
}

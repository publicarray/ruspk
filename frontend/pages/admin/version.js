import Layout from "../../components/layout-admin";
import Button from "../../components/button";
import DeleteBtn from "../../components/delete-btn";
import TablePaginate from "../../components/table-paginate";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatBoolean, API, API_VER } from '../../utils';
import { postJsonForm } from "../../utils";
import { useRouter } from 'next/router'

export default function VersionPage() {
    const url = `${API}/${API_VER}/version`
    const router = useRouter()
    const [data, setData] = useState([]);

    let del = async function (row, data) {
        const response = await fetch(`${url}/${row.values.id}`, {
            headers: {
                'Authorization': 'Bearer ' + localStorage.getItem("jwt")
            },
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
        { Header: 'Beta', accessor: row => {
            return row.report_url ? "Yes" : "No"
        }},
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

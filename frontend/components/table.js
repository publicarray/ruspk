import { useTable, useAsyncDebounce } from 'react-table'
import { fetchJson } from "../utils";
import useSWR from 'swr'
import React from 'react';

// https://react-table.tanstack.com/
export default function Table(props) {
    let pageIndex = props.pageIndex;
    if (!pageIndex || pageIndex <= 0) {
        pageIndex = 1
    }

    let pageSize = 15
    let sortBy = ""
    let filters = ""

    let { data, error } = useSWR(`${props.url}?page=${pageIndex}&size=${pageSize}`, fetchJson);
    let isLoading = !error && !data;
    let isError = !error;

    if (error) {
        console.error(error)
    }

    // for the preload table
    if (!props.setData) {
        return <></>;
    }

    // only update when the data changes
    React.useEffect(() => {
        if (!data) {
            props.setData([])
        } else {
            props.setData(data)
        }
    }, [data])

    const {
        getTableProps,
        getTableBodyProps,
        headerGroups,
        rows,
        prepareRow,
    } = useTable({
        columns: props.columns,
        data: props.data,
    })

    return (
        <div className="flex overflow-x-auto">
            <style jsx>{`
                td[data-header="architectures"] {
                    white-space: normal;
                }
            `}</style>
            <div className="w-full">
                <div className="bg-white dark:bg-gray-900 shadow-md rounded my-6">
                    <table className="w-full table-auto" {...getTableProps()}>
                        <thead>
                            {
                                headerGroups.map(headerGroup => (
                                    <tr className="bg-gray-200 text-black dark:bg-gray-600 dark:text-white uppercase text-sm leading-normal" {...headerGroup.getHeaderGroupProps()}>
                                        {
                                            headerGroup.headers.map(column => (
                                                <th className="py-3 px-6 text-left" {...column.getHeaderProps()}>{column.render('Header')}</th>
                                            ))
                                        }
                                    </tr>
                                ))
                            }
                        </thead>
                        <tbody className="text-gray-700 dark:text-gray-200 text-sm" {...getTableBodyProps()}>
                            {rows.map(row => {
                                prepareRow(row)
                                return (
                                    <tr className="border-b border-gray-200 hover:bg-gray-100 dark:border-gray-600 dark:hover:bg-gray-800" {...row.getRowProps()}>
                                        {row.cells.map(cell => {
                                            return (
                                                <td className="py-3 px-6 text-left sm:whitespace-nowrap" {...cell.getCellProps({'data-header': cell.column.render('Header').toLowerCase()})}>
                                                    {cell.render('Cell')}
                                                </td>
                                            )
                                        })}
                                    </tr>
                                )
                            })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    )
}

/*
 * Copyright (c) 2022 Yunshan Networks
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(dead_code)]

mod collector;
pub mod common;
mod config;
pub mod debug;
pub mod dispatcher;
#[cfg(target_os = "linux")]
mod ebpf;
#[cfg(target_os = "linux")]
mod ebpf_dispatcher;
mod error;
mod exception;
mod flow_generator;
mod handler;
mod integration_collector;
mod metric;
mod monitor;
mod platform;
mod policy;
mod rpc;
mod sender;
pub mod trident;
mod utils;

// for benchmarks
#[doc(hidden)]
pub use {
    common::endpoint::{
        EndpointData as _EndpointData, EndpointInfo as _EndpointInfo, FeatureFlags as _FeatureFlags,
    },
    common::enums::TcpFlags as _TcpFlags,
    common::feature as _feature,
    common::lookup_key::LookupKey as _LookupKey,
    common::platform_data::{IpSubnet as _IpSubnet, PlatformData as _PlatformData},
    common::policy::{Acl as _Acl, Cidr as _Cidr, IpGroupData as _IpGroupData},
    common::port_range::PortRange as _PortRange,
    flow_generator::flow_map::{
        _new_flow_map_and_receiver, _new_meta_packet, _reverse_meta_packet,
    },
    flow_generator::perf::l7_rrt::L7RrtCache as _L7RrtCache,
    flow_generator::perf::tcp::{
        TcpPerf as _TcpPerf, _benchmark_report, _benchmark_session_peer_seq_no_assert,
        _meta_flow_perf_update,
    },
    flow_generator::perf::FlowPerfCounter as _FlowPerfCounter,
    npb_pcap_policy::{
        NpbAction as _NpbAction, NpbTunnelType as _NpbTunnelType, TapSide as _TapSide,
    },
    policy::first_path::FirstPath as _FirstPath,
    policy::labeler::Labeler as _Labeler,
};

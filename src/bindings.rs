mod wasi_config_bindings {
    wit_bindgen::generate!({
        world: "config",
        inline: "
            package wassel:config;

            world config {
                include wasi:config/imports@0.2.0-rc.1;
            }
        ",
        generate_all,
    });
}

wit_bindgen::generate!({
    world: "http-plugin",
    pub_export_macro: true,
    default_bindings_module: "wassel_sdk::bindings",
    with: {
        "wassel:foundation/http-client": generate,
        "wassel:foundation/postgres": generate,

        "wasi:config/store@0.2.0-rc.1": wasi_config_bindings::wasi::config::store,

        "wasi:cli/stderr@0.2.10": wasip2::cli::stderr,
        "wasi:cli/stdin@0.2.10": wasip2::cli::stdin,
        "wasi:cli/stdout@0.2.10": wasip2::cli::stdout,
        "wasi:clocks/monotonic-clock@0.2.10": wasip2::clocks::monotonic_clock,
        "wasi:clocks/wall-clock@0.2.10": wasip2::clocks::wall_clock,
        "wasi:filesystem/preopens@0.2.10": wasip2::filesystem::preopens,
        "wasi:filesystem/types@0.2.10": wasip2::filesystem::types,
        "wasi:http/outgoing-handler@0.2.10": wasip2::http::outgoing_handler,
        "wasi:http/types@0.2.10": wasip2::http::types,
        "wasi:io/error@0.2.10": wasip2::io::error,
        "wasi:io/poll@0.2.10": wasip2::io::poll,
        "wasi:io/streams@0.2.10": wasip2::io::streams,
        "wasi:random/insecure-seed@0.2.10": wasip2::random::insecure_seed,
        "wasi:random/insecure@0.2.10": wasip2::random::insecure,
        "wasi:random/random@0.2.10": wasip2::random::random,
        "wasi:sockets/instance-network@0.2.10": wasip2::sockets::instance_network,
        "wasi:sockets/ip-name-lookup@0.2.10": wasip2::sockets::ip_name_lookup,
        "wasi:sockets/network@0.2.10": wasip2::sockets::network,
        "wasi:sockets/tcp-create-socket@0.2.10": wasip2::sockets::tcp_create_socket,
        "wasi:sockets/tcp@0.2.10": wasip2::sockets::tcp,
        "wasi:sockets/udp-create-socket@0.2.10": wasip2::sockets::udp_create_socket,
        "wasi:sockets/udp@0.2.10": wasip2::sockets::udp,

    },
});

pub use wasi_config_bindings::wasi::config as wasi_config;
pub use wasip2 as wasi;

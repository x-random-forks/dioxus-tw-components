
export function save_template(channel, nodes, tmpl_id) {
    channel.save_template(nodes, tmpl_id);
}
export function hydrate(channel, ids) {
    channel.hydrate(ids);
}
export function get_node(channel, id) {
    return channel.get_node(id);
}
export function initialize(channel, root, handler) {
    channel.initialize(root, handler);
}

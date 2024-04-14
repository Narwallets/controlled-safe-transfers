# Controlled Transfer Contract

Contract to hold tokens and transfer them only to pre-defined addresses.

A function-call key can be used to call the transfer methods, so a bot
can execute ft_transfer using a function-call-key (not a full access key),
and that key can only transfer to pre-defined addresses.

This makes the bot more secure, because it only has access to a function-call-key




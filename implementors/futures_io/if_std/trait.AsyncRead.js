(function() {var implementors = {};
implementors["gio"] = [{"text":"impl&lt;T:&nbsp;IsA&lt;IOStream&gt; + Unpin&gt; AsyncRead for IOStreamAsyncReadWrite&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;IsA&lt;InputStream&gt;&gt; AsyncRead for InputStreamAsyncBufRead&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;IsA&lt;PollableInputStream&gt;&gt; AsyncRead for InputStreamAsyncRead&lt;T&gt;","synthetic":false,"types":[]}];
implementors["gio_async_tls"] = [{"text":"impl AsyncRead for Connection","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
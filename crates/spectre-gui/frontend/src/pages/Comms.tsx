import { useState, useEffect } from 'react';
import { Radio, Users, Send } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';
import type { IdentityInfo, PeerInfo, SendRequest } from '@/types/comms';

export function Comms() {
  const [identity, setIdentity] = useState<IdentityInfo | undefined>();
  const [peers, setPeers] = useState<PeerInfo[]>([]);
  const [selectedPeer, setSelectedPeer] = useState<string>('');
  const [sendData, setSendData] = useState<string>('');
  const [sendResult, setSendResult] = useState<string | undefined>();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadIdentity();
    loadPeers();
  }, []);

  const loadIdentity = async () => {
    try {
      const id = await invoke<IdentityInfo | null>('get_identity');
      if (id) {
        setIdentity(id);
      }
    } catch (err) {
      console.error('Failed to load identity:', err);
    }
  };

  const loadPeers = async () => {
    try {
      const p = await invoke<PeerInfo[]>('list_peers');
      setPeers(p);
      if (p.length > 0 && !selectedPeer) {
        setSelectedPeer(p[0].name);
      }
    } catch (err) {
      console.error('Failed to load peers:', err);
    }
  };

  const handleSend = async () => {
    if (!selectedPeer || !sendData) return;

    setLoading(true);
    setSendResult(undefined);

    try {
      const request: SendRequest = {
        peer: selectedPeer,
        data: sendData,
      };
      const result = await invoke<string>('send_data', { request });
      setSendResult(result);
      setSendData('');
    } catch (err) {
      console.error('Failed to send data:', err);
      setSendResult(`Error: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Radio className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Secure Communications</h2>
      </div>

      <div className="grid gap-4 lg:grid-cols-2">
        {/* Identity */}
        <Card>
          <CardHeader>
            <CardTitle>Identity</CardTitle>
            <CardDescription>Your cryptographic identity (WRAITH protocol)</CardDescription>
          </CardHeader>
          <CardContent>
            {identity ? (
              <div className="space-y-2">
                <div>
                  <Label className="text-xs text-muted-foreground">Name</Label>
                  <p className="text-sm font-medium">{identity.name}</p>
                </div>
                <div>
                  <Label className="text-xs text-muted-foreground">Fingerprint</Label>
                  <p className="font-mono text-xs">{identity.fingerprint}</p>
                </div>
                <div>
                  <Label className="text-xs text-muted-foreground">Created</Label>
                  <p className="text-xs">{new Date(identity.created).toLocaleString()}</p>
                </div>
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">No identity configured</p>
            )}
          </CardContent>
        </Card>

        {/* Peers */}
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <div>
                <CardTitle>
                  <Users className="mr-2 inline h-4 w-4" />
                  Trusted Peers
                </CardTitle>
                <CardDescription>{peers.length} configured</CardDescription>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {peers.map((peer) => (
                <div
                  key={peer.fingerprint}
                  className="flex items-center justify-between rounded-lg border border-border p-3"
                >
                  <div>
                    <p className="text-sm font-medium">{peer.name}</p>
                    <p className="font-mono text-xs text-muted-foreground">
                      {peer.fingerprint.slice(0, 20)}...
                    </p>
                  </div>
                  <Badge variant={peer.status === 'online' ? 'default' : 'secondary'}>
                    {peer.status}
                  </Badge>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Send Data */}
      <Card>
        <CardHeader>
          <CardTitle>
            <Send className="mr-2 inline h-4 w-4" />
            Send Encrypted Data
          </CardTitle>
          <CardDescription>Send secure encrypted data to a trusted peer</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="peer">Recipient</Label>
              <Select value={selectedPeer} onValueChange={setSelectedPeer}>
                <SelectTrigger id="peer">
                  <SelectValue placeholder="Select peer" />
                </SelectTrigger>
                <SelectContent>
                  {peers.map((peer) => (
                    <SelectItem key={peer.fingerprint} value={peer.name}>
                      {peer.name} ({peer.status})
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="data">Data</Label>
            <textarea
              id="data"
              value={sendData}
              onChange={(e) => setSendData(e.target.value)}
              placeholder="Enter data to send..."
              className="w-full rounded-md border border-input bg-background p-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
              rows={6}
            />
          </div>

          <div className="flex items-center gap-2">
            <Button onClick={handleSend} disabled={loading || !selectedPeer || !sendData}>
              {loading ? 'Sending...' : 'Send Encrypted'}
            </Button>
          </div>

          {sendResult && (
            <div className="rounded-lg bg-muted p-3 text-sm">{sendResult}</div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

// Default export for lazy loading
export default Comms;

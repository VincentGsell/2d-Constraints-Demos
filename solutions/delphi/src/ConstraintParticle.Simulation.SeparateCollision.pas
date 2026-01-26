unit ConstraintParticle.Simulation.SeparateCollision;

interface

uses
  System.Types,
  System.SysUtils,
  System.UITypes,
  System.Generics.Collections,
  ConstraintParticle;

type
  TSeparateCollisionSimulation = class(TConstraintParticleSimulation)
  private
    FIterations: Integer;
    FBallRadius: Single;
    class function RandomColor: TAlphaColor; static;
  public
    constructor Create(aCount: Integer; aBallRadius: Single; aMainRadius: Single;
      const aStartPos: TPointF; aMainColor: TAlphaColor;
      aUseRandomColors: Boolean = True);
    procedure Update(const aMousePos: TPointF); override;

    property Iterations: Integer read FIterations write FIterations;
  end;

implementation

{ TSeparateCollisionSimulation }

class function TSeparateCollisionSimulation.RandomColor: TAlphaColor;
begin
  Result := TAlphaColorF.Create(Random(255) / 255, Random(255) / 255, Random(255) / 255, 1).ToAlphaColor;
end;

constructor TSeparateCollisionSimulation.Create(aCount: Integer; aBallRadius: Single;
  aMainRadius: Single; const aStartPos: TPointF; aMainColor: TAlphaColor;
  aUseRandomColors: Boolean);
var
  i, xj, yj: Integer;
begin
  inherited Create;

  FIterations := 1;
  FBallRadius := aBallRadius;

  FMainParticle.Pos := aStartPos;
  FMainParticle.Radius := aMainRadius;
  FMainParticle.Color := aMainColor;

  SetLength(FParticles, aCount);
  xj := 0;
  yj := 0;
  for i := 0 to aCount - 1 do
  begin
    if (i > 0) and (i mod 50 = 0) then
    begin
      xj := 0;
      Inc(yj);
    end;
    FParticles[i].Pos := PointF(aStartPos.X + 100 + (xj * aBallRadius),
                                 aStartPos.Y + (yj * aBallRadius));
    FParticles[i].Radius := aBallRadius;
    if aUseRandomColors then
      FParticles[i].Color := RandomColor
    else
      FParticles[i].Color := TAlphaColors.White;
    Inc(xj);
  end;
end;

procedure TSeparateCollisionSimulation.Update(const aMousePos: TPointF);
var
  toNext, lOffset: TPointF;
  i, j, k, iter: Integer;
  lRadius: Single;
  hash: TSpatialHash;
  nearby: TList<Integer>;
begin
  FMainParticle.Pos := aMousePos;

  //Constraint for main circle
  for i := 0 to High(FParticles) do
  begin
    toNext := FMainParticle.Pos - FParticles[i].Pos;
    if toNext.Length < FMainParticle.Radius + FParticles[i].Radius then
    begin
      toNext.SetLength(FMainParticle.Radius + FParticles[i].Radius);
      lOffset := FMainParticle.Pos - FParticles[i].Pos - toNext;
      FParticles[i].Pos := FParticles[i].Pos + lOffset;
    end;
  end;

  //Separate balls - spatial hash O(n) avec iterations multiples
  hash := TSpatialHash.Create(FBallRadius * 2);
  nearby := TList<Integer>.Create;
  try
    for iter := 1 to FIterations do
    begin
      hash.Clear;
      for i := 0 to High(FParticles) do
        hash.Insert(i, FParticles[i].Pos);

      for i := 0 to High(FParticles) do
      begin
        nearby.Clear;
        hash.GetNearby(FParticles[i].Pos, nearby);
        for k := 0 to nearby.Count - 1 do
        begin
          j := nearby[k];
          if j <= i then
            Continue;

          toNext := FParticles[j].Pos - FParticles[i].Pos;
          lRadius := FParticles[j].Radius + FParticles[i].Radius;
          if toNext.Length <= lRadius then
          begin
            toNext.SetLength(lRadius);
            lOffset := FParticles[j].Pos - FParticles[i].Pos - toNext;
            lOffset := lOffset / 2;
            FParticles[i].Pos := FParticles[i].Pos + lOffset;
            FParticles[j].Pos := FParticles[j].Pos - lOffset;
          end;
        end;
      end;
    end;
  finally
    nearby.Free;
    hash.Free;
  end;
end;

end.
